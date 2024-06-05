use fs_err as fs;
use progenitor::TypeImpl;
use progenitor::TypePatch;
use quote::quote;
// use schemars::schema::Metadata;
use std::io::Write;

fn api_spec_path() -> std::path::PathBuf {
    manifest_dir().join("binance-api-swagger.yaml")
}

fn manifest_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("Always provided in build.rs. qed"),
    )
}
fn dest_path() -> std::path::PathBuf {
    manifest_dir().join("src").join("codegen.rs")
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", api_spec_path().display());
    println!("cargo:rerun-if-changed={}", dest_path().display());

    let res = run_progenitor();
    
    if let Err(cause) = res {
        while let Some(cause) = cause.source() {
            eprintln!("Caused by {:?}", cause);
        }
        eprintln!("");
        Err(cause)
    } else {
        Ok(())
    }
}


fn run_progenitor() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(api_spec_path())?;
    let de = serde_yaml::Deserializer::from_reader(file);
    let spec = serde_path_to_error::deserialize(de)?;
    let schema_json = r#"
	{
		"required": ["id", "objectName" ],
		"properties": {
		  "id":{
			"type": "integer"
			},
		  "objectName": {
			"type": "string"
		  }
		},
		"type": "object"
	}
	"#;

    let _schema_universal_id_repl: schemars::schema::SchemaObject =
        serde_json::from_str(schema_json).unwrap();

    let patchty = TypePatch::default();
    // patchty.with_derive("Display");
    // let patchty = patchty;
    let mut settings = progenitor::GenerationSettings::default();
    settings
        .with_inner_type(quote! {
            crate::AuthProvider
        })
        .with_pre_hook(quote! {
            crate::pre_hook
        })
        .with_post_hook(quote! {
            crate::post_hook
        })
        // .with_derive("::schemars::JsonSchema")
        // .with_patch("ExportVoucherZipSevQuery", &patchty)
        // .with_patch("ExportContactSevQuery", &patchty)
        // .with_patch("ExportInvoiceZipSevQuery", &patchty)
        // .with_patch("ExportCreditNoteSevQuery", &patchty)
        // .with_patch("ExportTransactionsSevQuery", &patchty)
        // .with_patch("ReportOrderSevQuery", &patchty)
        // .with_patch("ExportVoucherSevQuery", &patchty)
        // .with_patch("ExportInvoiceSevQuery", &patchty)
        .with_conversion(
            schemars::schema::SchemaObject {
                instance_type: Some(schemars::schema::InstanceType::String.into()),
                format: Some("binary".to_owned()),
                metadata: None,
                ..Default::default()
            },
            "crate::Blob",
            [TypeImpl::Display].into_iter(),
        )
        .with_conversion(
            schemars::schema::SchemaObject {
                instance_type: Some(schemars::schema::InstanceType::String.into()),
                format: Some("date".to_owned()),
                metadata: None,
                ..Default::default()
            },
            "crate::DateRfc3339",
            [TypeImpl::Display].into_iter(),
        )
        .with_conversion(
            schemars::schema::SchemaObject {
                instance_type: Some(schemars::schema::InstanceType::String.into()),
                format: Some("date-time".to_owned()),
                metadata: None,
                ..Default::default()
            },
            "crate::DateTimeRfc3339",
            [TypeImpl::Display].into_iter(),
        )
        // .with_conversion(
        //     schemars::schema::SchemaObject {
        //         instance_type: Some(schemars::schema::InstanceType::String.into()),
        //         format: Some("iso-currency".to_owned()),
        //         metadata: None,
        //         ..Default::default()
        //     },
        //     "crate::Currency",
        //     [TypeImpl::Display].into_iter(),
        // )
        // .with_conversion(
        //     schema_universal_id_repl, // doesn't work yet
        //     "crate::IdUni",
        //     [TypeImpl::Display].into_iter(),
        // )
        ;

    let mut generator = progenitor::Generator::new(&settings);

    let tokens = generator.generate_tokens(&spec)?;
    let ast = syn::parse2(tokens)?;
    let content = prettyplease::unparse(&ast);

    let content = String::from(
        r###"
    #![allow(dead_code, deprecated)]
    #![allow(clippy::needless_lifetimes, clippy::too_many_arguments, clippy::from_str_radix_10, clippy::vec_init_then_push)]
    
    use crate::datetimerfc3339::*;
    "###,
    ) + content.as_str();

    if cfg!(feature = "genapi") {
        // let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR is set for build.rs at runtime. qed")).to_path_buf().join("codegen.rs");
        let out_file = dest_path();

        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(out_file)?;
        out_file.write_all(content.as_bytes())?;
    } else {
        println!("cargo:warning=Skipping codegen dump to file.")
    }
    Ok(())
}
