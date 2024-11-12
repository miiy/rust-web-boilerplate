use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// vite backend integration
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

// importedChunks
//
// import type { Manifest, ManifestChunk } from 'vite'
//
// export default function importedChunks(
//   manifest: Manifest,
//   name: string,
// ): ManifestChunk[] {
//   const seen = new Set<string>()
//
//   function getImportedChunks(chunk: ManifestChunk): ManifestChunk[] {
//     const chunks: ManifestChunk[] = []
//     for (const file of chunk.imports ?? []) {
//       const importee = manifest[file]
//       if (seen.has(file)) {
//         continue
//       }
//       seen.add(file)
//
//       chunks.push(...getImportedChunks(importee))
//       chunks.push(importee)
//     }
//
//     return chunks
//   }
//
//   return getImportedChunks(manifest[name])
// }

type ManifestMap = HashMap<String, ManifestChunk>;

struct Manifest {
    data: ManifestMap,
}

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

impl FromStr for Manifest {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: serde_json::from_str::<ManifestMap>(s)?,
        })
    }
}

impl Manifest {
    pub fn chunk(&self, name: &str) -> Option<&ManifestChunk> {
        manifest_chunk(&self.data, name)
    }

    pub fn imported_chunks(&self, name: &str) -> Vec<ManifestChunk> {
        imported_chunks(&self.data, name)
    }
}

fn manifest_chunk<'a>(manifest: &'a ManifestMap, name: &str) -> Option<&'a ManifestChunk> {
    manifest.get(name)
}

fn imported_chunks(manifest: &ManifestMap, name: &str) -> Vec<ManifestChunk> {
    let mut seen: HashSet<String> = HashSet::new();

    fn get_imported_chunks(
        chunk: &ManifestChunk,
        manifest: &ManifestMap,
        seen: &mut HashSet<String>,
    ) -> Vec<ManifestChunk> {
        let mut chunks: Vec<ManifestChunk> = Vec::new();
        if let Some(imports) = &chunk.imports {
            for file in imports.iter() {
                if seen.contains(file) {
                    continue;
                }
                seen.insert(file.clone());

                match manifest.get(file) {
                    Some(importee) => {
                        chunks.extend(get_imported_chunks(&importee, manifest, seen));
                        chunks.push(importee.clone());
                    }
                    None => (),
                }
            }
        }
        chunks
    }

    match manifest.get(name) {
        Some(chunk) => get_imported_chunks(chunk, manifest, &mut seen),
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Manifest {
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
        Manifest::from_str(manifest_json).unwrap()
    }

    #[test]
    fn test_parse_manifest() {
        let manifest = setup();
        assert_eq!(manifest.data.len(), 5);
    }

    #[test]
    fn test_manifest_chunk() {
        let manifest = setup();
        let manifest_chunk = manifest.chunk("views/foo.js").unwrap();
        assert_eq!(&manifest_chunk.file, "assets/foo-BRBmoGS9.js");
    }

    #[test]
    fn test_manifest_imported_chunks() {
        let manifest = setup();
        let chunks = manifest.imported_chunks("views/foo.js");
        let chunk = chunks.first().unwrap();
        assert_eq!(chunk.file, "assets/shared-B7PI925R.js");

        let chunks = manifest.imported_chunks("views/bar.js");
        let chunk = chunks.first().unwrap();
        assert_eq!(chunk.file, "assets/shared-B7PI925R.js");

        let chunks = manifest.imported_chunks("baz.js");
        assert_eq!(chunks.len(), 0);
    }
}
