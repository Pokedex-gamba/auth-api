use utoipa::Modify;

pub struct AutoTagAddon;

impl Modify for AutoTagAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        for path in openapi.paths.paths.values_mut() {
            for operation in path.operations.values_mut() {
                let tags = operation.tags.take().unwrap_or_default();

                let mut new_tags = tags
                    .into_iter()
                    .filter(|t| !t.starts_with("crate::"))
                    .collect::<Vec<_>>();
                new_tags.push("All routes".into());

                operation.tags = Some(new_tags);
            }
        }
    }
}
