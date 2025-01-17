use crate::components::{home::Home, week::Week};
use chrono::{Datelike, Utc};
use twin::news::Month;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,

  #[at("/latest")]
  Latest,

  #[at("/:year/:month/:day")]
  Week { year: u16, month: Month, day: u8 },

  #[not_found]
  #[at("/404")]
  NotFound,
}

impl Route {
  pub fn switch(&self) -> Html {
    let component = match *self {
      Route::Home => html! { <Home /> },

      Route::Latest => html! { <Week /> },

      Route::Week { year, month, day } => html! { <Week year={year} month={month} day={day} /> },

      Route::NotFound => todo!(),
    };

    Self::wrap_component(component)
  }

  fn wrap_component(component: Html) -> Html {
    let now = Utc::now();

    html! {
      <div>
        <section class="hero is-success">
          <div class="hero-body has-text-centered">
            <a href="/">
              <h1 class="title">{ "This Week in Neovim" }</h1>
            </a>
          </div>

        </section>

        {component}

        <footer class="footer has-text-centered">
          <p class="block">
            { "Made by " }
            <a href={"https://github.com/phaazon"}>{ "@phaazon" }</a>
            { " and contributors. " }
            <a href="https://github.com/phaazon/this-week-in-neovim.org/blob/master/LICENSE">
              { now.year() }
              { " BSD-3 New Clause " }
            </a>
          </p>

          <p class="block">
            <a href="https://github.com/phaazon/this-week-in-neovim.org">
              <span class="icon-text has-text-link">
                <span class="icon">
                  <i class="fa-brands fa-github"></i>
                </span>
                <span>{ "TWiN" }</span>
              </span>
            </a>
            { " | " }
            <a href="https://rust-lang.org">
              <span class="icon-text has-text-link">
                <span class="icon">
                  <i class="fa-brands fa-rust"></i>
                </span>
                <span>{ "Rust" }</span>
              </span>
            </a>
            { " | " }
            <a href="https://rocket.rs">
              <span class="icon-text has-text-link">
                <span class="icon">
                  <i class="fa-solid fa-shuttle-space"></i>
                </span>
                <span>{ "rocket-rs" }</span>
              </span>
            </a>
            { " | " }
            <a href="https://yew.rs">
              <span class="icon-text has-text-link">
                <span class="icon">
                  <i class="fa-solid fa-y"></i>
                </span>
                <span>{ "Yew" }</span>
              </span>
            </a>
            { " | " }
            <a href="/api/rss">
              <span class="icon-text has-text-danger">
                <span class="icon">
                  <i class="fa-solid fa-rss"></i>
                </span>
                <span>
                { "RSS feed" }
                </span>
              </span>
            </a>
          </p>
        </footer>
      </div>
    }
  }
}
