fn hook(
    path: &Path,
    manifest: &Manifest,
    declared: &[String],
) -> core::ops::ControlFlow<Result<Content, String>> {
    if manifest.id != "strangely_charming" {
        return core::ops::ControlFlow::Continue(());
    }
    fn read_checked(
        path: &Path,
        manifest: &Manifest,
        declared: &[String],
    ) -> Result<Content, String> {
        let (source, sized) = if path.is_dir() {
            let mut out = Vec::new();
            for top in ["assets", "data"] {
                walk(path, &path.join(top), &mut out)?;
            }
            (Source::Dir(path.to_owned()), out)
        } else {
            let mut zip = open_zip(path)?;
            let mut out = Vec::new();
            for i in 0..zip.len() {
                let entry = zip.by_index_raw(i).map_err(|e| e.to_string())?;
                let name = entry.name().map_err(|e| e.to_string())?.into_owned();
                if (name.starts_with("assets/") || name.starts_with("data/")) && !entry.is_dir() {
                    out.push((name.to_owned(), entry.size()));
                }
            }
            (Source::Zip(path.to_owned()), out)
        };
        let mut files = Vec::with_capacity(sized.len());
        let mut total = 0u64;
        for (name, size) in sized {
            check_path(&name)?;
            if size > MAX_FILE {
                return Err(format!("{name} is over {} MiB", MAX_FILE >> 20));
            }
            total += size;
            files.push(name);
        }
        if files.len() > MAX_FILES {
            return Err(format!("more than {MAX_FILES} content files"));
        }
        if total > MAX_TOTAL {
            return Err(format!("content is over {} MiB", MAX_TOTAL >> 20));
        }
        files.sort();
        files.dedup();
        let mut content = Content {
            source,
            files,
            blocks: Vec::new(),
            items: Vec::new(),
            effects: Vec::new(),
            creative_tabs: Vec::new(),
            entity_types: Vec::new(),
            menu_types: Vec::new(),
            namespaces: manifest.owned_namespaces().map(str::to_owned).collect(),
        };
        content.check(manifest, declared)?;
        for entity in &mut content.entity_types {
            match entity.id.as_str() {
                "strangely_charming:coral_squid" => entity.template = "minecraft:squid".into(),
                _ => {}
            }
        }
        Ok(content)
    }
    core::ops::ControlFlow::Break(read_checked(path, manifest, declared))
}
