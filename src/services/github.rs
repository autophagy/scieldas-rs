use crate::scieldas::{Scield, ScieldRequest, StateScield, TextScield};
use crate::utils::get_payload;
use reqwest::Client;
use rocket::request::FromParam;
use rocket::State;
use std::str::FromStr;

const GITHUB_API_URL: &str = "https://api.github.com";

const WATCHERS_SCIELD: TextScield = TextScield {
    prefix: "Watchers",
    suffix: None,
};

const FORKS_SCIELD: TextScield = TextScield {
    prefix: "Watchers",
    suffix: None,
};

const STARS_SCIELD: TextScield = TextScield {
    prefix: "Stars",
    suffix: None,
};

const FOLLOWERS_SCIELD: TextScield = TextScield {
    prefix: "Followers",
    suffix: None,
};

const LATEST_RELEASE_SCIELD: TextScield = TextScield {
    prefix: "Release",
    suffix: None,
};

const ISSUES_SCIELD: TextScield = TextScield {
    prefix: "Issues",
    suffix: None,
};

const PULL_REQUESTS_SCIELD: TextScield = TextScield {
    prefix: "Pull Requests",
    suffix: None,
};

enum WorkflowState {
    Passing,
    Failing,
    Unknown,
}

#[derive(Debug)]
struct ParseWorkflowStateError;

impl FromStr for WorkflowState {
    type Err = ParseWorkflowStateError;
    fn from_str(s: &str) -> Result<WorkflowState, ParseWorkflowStateError> {
        match s {
            "success" => Ok(WorkflowState::Passing),
            "failure" => Ok(WorkflowState::Failing),
            _ => Ok(WorkflowState::Unknown),
        }
    }
}

impl ToString for WorkflowState {
    fn to_string(&self) -> String {
        match &self {
            WorkflowState::Passing => "Passing".to_string(),
            WorkflowState::Failing => "Failing".to_string(),
            WorkflowState::Unknown => "Unknown".to_string(),
        }
    }
}

const WORKFLOW_SCIELD: StateScield = StateScield {
    prefix: Some("Build"),
    suffix: None,
};

enum OpenState {
    All,
    Open,
    Closed,
}

impl OpenState {
    fn to_search_param(&self) -> &str {
        match self {
            OpenState::All => "",
            OpenState::Open => "+is:open",
            OpenState::Closed => "+is:closed",
        }
    }
}

#[derive(Debug)]
enum OpenStateError {
    InvalidOpenState,
}

impl<'r> FromParam<'r> for OpenState {
    type Error = OpenStateError;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "all" => Ok(OpenState::All),
            "open" => Ok(OpenState::Open),
            "closed" => Ok(OpenState::Closed),
            _ => Err(OpenStateError::InvalidOpenState),
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        watchers,
        forks,
        stars,
        followers,
        latest_release,
        issues,
        pull_requests,
        workflow
    ]
}

#[get("/watchers/<owner>/<repo>")]
async fn watchers(
    client: &State<Client>,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let watchers = get_payload(client, &request_url)
        .await?
        .get("subscribers_count")?
        .as_f64()?;

    Some(Scield {
        scield: WATCHERS_SCIELD,
        value: watchers,
        filetype: repo.filetype,
    })
}

#[get("/forks/<owner>/<repo>")]
async fn forks(
    client: &State<Client>,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let forks = get_payload(client, &request_url)
        .await?
        .get("forks_count")?
        .as_f64()?;

    Some(Scield {
        scield: FORKS_SCIELD,
        value: forks,
        filetype: repo.filetype,
    })
}

#[get("/stars/<owner>/<repo>")]
async fn stars(
    client: &State<Client>,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let stars = get_payload(client, &request_url)
        .await?
        .get("stargazers_count")?
        .as_f64()?;

    Some(Scield {
        scield: STARS_SCIELD,
        value: stars,
        filetype: repo.filetype,
    })
}

#[get("/followers/<user>")]
async fn followers(
    client: &State<Client>,
    user: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!("{}/users/{}", GITHUB_API_URL, user.body);

    let followers = get_payload(client, &request_url)
        .await?
        .get("followers")?
        .as_f64()?;

    Some(Scield {
        scield: FOLLOWERS_SCIELD,
        value: followers,
        filetype: user.filetype,
    })
}

#[get("/latest_release/<owner>/<repo>")]
async fn latest_release(
    client: &State<Client>,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<String, TextScield>> {
    let request_url = format!(
        "{}/repos/{}/{}/releases/latest",
        GITHUB_API_URL, owner, repo.body
    );

    let latest_release = String::from(
        get_payload(client, &request_url)
            .await?
            .get("tag_name")?
            .as_str()?,
    );

    Some(Scield {
        scield: LATEST_RELEASE_SCIELD,
        value: latest_release,
        filetype: repo.filetype,
    })
}

#[get("/issues/<state>/<owner>/<repo>")]
async fn issues(
    client: &State<Client>,
    state: OpenState,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!(
        "{}/search/issues?q=repo:{}/{}+is:issue{}",
        GITHUB_API_URL,
        owner,
        repo.body,
        state.to_search_param()
    );

    let issues = get_payload(client, &request_url)
        .await?
        .get("total_count")?
        .as_f64()?;

    Some(Scield {
        scield: ISSUES_SCIELD,
        value: issues,
        filetype: repo.filetype,
    })
}

#[get("/pull_requests/<state>/<owner>/<repo>")]
async fn pull_requests(
    client: &State<Client>,
    state: OpenState,
    owner: &str,
    repo: ScieldRequest<String>,
) -> Option<Scield<f64, TextScield>> {
    let request_url = format!(
        "{}/search/issues?q=repo:{}/{}+is:pr{}",
        GITHUB_API_URL,
        owner,
        repo.body,
        state.to_search_param()
    );

    let pulls = get_payload(client, &request_url)
        .await?
        .get("total_count")?
        .as_f64()?;

    Some(Scield {
        scield: PULL_REQUESTS_SCIELD,
        value: pulls,
        filetype: repo.filetype,
    })
}

#[get("/workflow/<owner>/<repo>/<workflow>/<branch>")]
async fn workflow(
    client: &State<Client>,
    owner: &str,
    repo: &str,
    workflow: &str,
    branch: ScieldRequest<String>,
) -> Option<Scield<WorkflowState, StateScield>> {
    let request_url = format!(
        "{}/repos/{}/{}/actions/workflows/{}/runs?branch={}&per_page=1&status=completed",
        GITHUB_API_URL, owner, repo, workflow, branch.body
    );

    let payload = get_payload(client, &request_url).await?;

    let status = if payload.get("total_count")?.as_i64()? == 0 {
        String::from("unknown")
    } else {
        payload
            .pointer("/workflow_runs/0/conclusion")?
            .as_str()?
            .to_string()
    };

    match WorkflowState::from_str(&status) {
        Ok(value) => Some(Scield {
            scield: WORKFLOW_SCIELD,
            value,
            filetype: branch.filetype,
        }),
        Err(_) => None,
    }
}
