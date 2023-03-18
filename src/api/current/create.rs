use crate::{Error, models::Repository, FromResponse};

use super::CurrentAuthHandler;

#[derive(serde::Serialize)]
pub struct CreateUserRepositoryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r CurrentAuthHandler<'octo>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignore_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

impl<'octo, 'r> CreateUserRepositoryBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r CurrentAuthHandler<'octo>, name: impl Into<String>) -> Self {
        Self {
            handler,
            name: name.into(),
            description: None,
            homepage: None,
            has_issues: None,
            has_projects: None,
            has_wiki: None,
            auto_init: None,
            gitignore_template: None,
            license_template: None,
            private: None,
        }
    }

    /// A short description of the repository.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// A URL with more information about the repository.
    pub fn homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    /// Whether issues are enabled.
    pub fn has_issues(mut self, has_issues: impl Into<bool>) -> Self {
        self.has_issues = Some(has_issues.into());
        self
    }

    /// Whether projects are enabled
    pub fn has_projects(mut self, has_projects: impl Into<bool>) -> Self {
        self.has_projects = Some(has_projects.into());
        self
    }

    /// Whether the wiki is enabled
    pub fn has_wiki(mut self, has_wiki: impl Into<bool>) -> Self {
        self.has_wiki = Some(has_wiki.into());
        self
    }

    /// Whether the repository is initialized with a minimal README.
    pub fn auto_init(mut self, auto_init: impl Into<bool>) -> Self {
        self.auto_init = Some(auto_init.into());
        self
    }

    /// The desired language or platform to apply to the .gitignore.
    pub fn gitignore_template(mut self, gitignore_template: impl Into<String>) -> Self {
        self.gitignore_template = Some(gitignore_template.into());
        self
    }

    /// The license keyword of the open source license for this repository.
    pub fn license_template(mut self, license_template: impl Into<String>) -> Self {
        self.license_template = Some(license_template.into());
        self
    }

    /// Whether the repository is private.
    pub fn private(mut self, private: impl Into<bool>) -> Self {
        self.private = Some(private.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Repository, Error> {
        let url = "user/repos";
        let request = self
            .handler
            .crab
            .client
            .post(self.handler.crab.absolute_url(url)?)
            .body(serde_json::to_string(&self).unwrap())
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.github+json"
            );

        let response = self.handler.crab.execute(request).await?;

        Repository::from_response(response).await
    }

}
