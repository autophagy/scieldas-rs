use crate::scieldas::{Scield, ScieldRequest, TextScield};
use crate::utils::{get_payload, readable_number};
use rocket::request::FromParam;

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
        pull_requests
    ]
}

#[get("/watchers/<owner>/<repo>")]
async fn watchers(owner: &str, repo: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let watchers = get_payload(&request_url)
        .await?
        .get("subscribers_count")?
        .as_f64()?;

    Some(Scield {
        scield: WATCHERS_SCIELD,
        value: readable_number(watchers),
        filetype: repo.filetype,
    })
}

#[get("/forks/<owner>/<repo>")]
async fn forks(owner: &str, repo: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let forks = get_payload(&request_url)
        .await?
        .get("forks_count")?
        .as_f64()?;

    Some(Scield {
        scield: FORKS_SCIELD,
        value: readable_number(forks),
        filetype: repo.filetype,
    })
}

#[get("/stars/<owner>/<repo>")]
async fn stars(owner: &str, repo: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/repos/{}/{}", GITHUB_API_URL, owner, repo.body);

    let stars = get_payload(&request_url)
        .await?
        .get("stargazers_count")?
        .as_f64()?;

    Some(Scield {
        scield: STARS_SCIELD,
        value: readable_number(stars),
        filetype: repo.filetype,
    })
}

#[get("/followers/<user>")]
async fn followers(user: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/users/{}", GITHUB_API_URL, user.body);

    let followers = get_payload(&request_url)
        .await?
        .get("followers")?
        .as_f64()?;

    Some(Scield {
        scield: FOLLOWERS_SCIELD,
        value: readable_number(followers),
        filetype: user.filetype,
    })
}

#[get("/latest_release/<owner>/<repo>")]
async fn latest_release(owner: &str, repo: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!(
        "{}/repos/{}/{}/releases/latest",
        GITHUB_API_URL, owner, repo.body
    );

    let latest_release = String::from(get_payload(&request_url).await?.get("tag_name")?.as_str()?);

    Some(Scield {
        scield: LATEST_RELEASE_SCIELD,
        value: latest_release,
        filetype: repo.filetype,
    })
}

#[get("/issues/<state>/<owner>/<repo>")]
async fn issues(state: OpenState, owner: &str, repo: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!(
        "{}/search/issues?q=repo:{}/{}+is:issue{}",
        GITHUB_API_URL,
        owner,
        repo.body,
        state.to_search_param()
    );

    let issues = get_payload(&request_url)
        .await?
        .get("total_count")?
        .as_f64()?;

    Some(Scield {
        scield: ISSUES_SCIELD,
        value: readable_number(issues),
        filetype: repo.filetype,
    })
}

#[get("/pull_requests/<state>/<owner>/<repo>")]
async fn pull_requests(
    state: OpenState,
    owner: &str,
    repo: ScieldRequest,
) -> Option<Scield<TextScield>> {
    let request_url = format!(
        "{}/search/issues?q=repo:{}/{}+is:pr{}",
        GITHUB_API_URL,
        owner,
        repo.body,
        state.to_search_param()
    );

    let pulls = get_payload(&request_url)
        .await?
        .get("total_count")?
        .as_f64()?;

    Some(Scield {
        scield: PULL_REQUESTS_SCIELD,
        value: readable_number(pulls),
        filetype: repo.filetype,
    })
}
