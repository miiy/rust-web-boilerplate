use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

//
// https://cn.vite.dev/guide/backend-integration.html
// Manifest
// src: vite/dist/node/index.d.ts
//
// type Manifest = Record<string, ManifestChunk>;
// interface ManifestChunk {
//     src?: string;
//     file: string;
//     css?: string[];
//     assets?: string[];
//     isEntry?: boolean;
//     name?: string;
//     isDynamicEntry?: boolean;
//     imports?: string[];
//     dynamicImports?: string[];
// }
type Manifest = HashMap<String, ManifestChunk>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ManifestChunk {
    src: Option<String>,
    file: String,
    css: Option<Vec<String>>,
    assets: Option<Vec<String>>,
    is_entry: Option<bool>,
    name: Option<String>,
    is_dynamic_entry: Option<bool>,
    imports: Option<Vec<String>>,
    dynamic_imports: Option<Vec<String>>,
}

/*
import type { Manifest, ManifestChunk } from 'vite'

export default function importedChunks(
  manifest: Manifest,
  name: string,
): ManifestChunk[] {
  const seen = new Set<string>()

  function getImportedChunks(chunk: ManifestChunk): ManifestChunk[] {
    const chunks: ManifestChunk[] = []
    for (const file of chunk.imports ?? []) {
      const importee = manifest[file]
      if (seen.has(file)) {
        continue
      }
      seen.add(file)

      chunks.push(...getImportedChunks(importee))
      chunks.push(importee)
    }

    return chunks
  }

  return getImportedChunks(manifest[name])
}
*/

fn imported_chunks(manifest: &Manifest, name: &str) -> Vec<ManifestChunk> {
    let mut seen: HashSet<String> = HashSet::new();

    let chunk = match manifest.get(name) {
        Some(chunk) => chunk,
        None => return Vec::new(),
    };

    fn get_imported_chunks(chunk: &ManifestChunk, manifest: &Manifest, seen: &mut HashSet<String>) -> Vec<ManifestChunk> {
        let mut chunks: Vec<ManifestChunk> = Vec::new();
        if let Some(imports) = &chunk.imports {
            for file in imports.iter() {
                let importee_option = manifest.get(file);

                if seen.contains(file) {
                    continue;
                }
                seen.insert(file.clone());

                if importee_option.is_some() {
                    let importee = importee_option.unwrap();
                    chunks.extend(get_imported_chunks(&importee, manifest, seen));
                    chunks.push(importee.clone());
                }
            }
        }
        chunks
    }

    get_imported_chunks(chunk, manifest, &mut seen)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manifest() {
        let manifest_json = r#"
{
  "_shared-B7PI925R.js": {
    "file": "assets/shared-B7PI925R.js",
    "name": "shared",
    "css": ["assets/shared-ChJ_j-JJ.css"]
  },
  "_shared-ChJ_j-JJ.css": {
    "file": "assets/shared-ChJ_j-JJ.css",
    "src": "_shared-ChJ_j-JJ.css"
  },
  "baz.js": {
    "file": "assets/baz-B2H3sXNv.js",
    "name": "baz",
    "src": "baz.js",
    "isDynamicEntry": true
  },
  "views/bar.js": {
    "file": "assets/bar-gkvgaI9m.js",
    "name": "bar",
    "src": "views/bar.js",
    "isEntry": true,
    "imports": ["_shared-B7PI925R.js"],
    "dynamicImports": ["baz.js"]
  },
  "views/foo.js": {
    "file": "assets/foo-BRBmoGS9.js",
    "name": "foo",
    "src": "views/foo.js",
    "isEntry": true,
    "imports": ["_shared-B7PI925R.js"],
    "css": ["assets/foo-5UjPuW-k.css"]
  }
}
"#;
        let manifest: Manifest = serde_json::from_str(manifest_json).unwrap();
        println!("{:?}", manifest);
        let manifest_chunks = imported_chunks(&manifest, "views/foo.js");
        println!("{:?}", manifest_chunks)
    }
}