use yew::prelude::*;
use yew_router::prelude::*;

use crate::{hooks::use_user_context, pages::Route, types::UserInfo};

#[function_component(Header)]
pub fn header_component() -> Html {
    let user_ctx = use_user_context();

    let onclick = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
          <div class="navbar-brand">
            <p class="navbar-item"> { "Cuscuz Flag" } </p>

            <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
              <span aria-hidden="true"></span>
              <span aria-hidden="true"></span>
              <span aria-hidden="true"></span>
            </a>
          </div>

          <div id="navbarBasicExample" class="navbar-menu">
            <div class="navbar-start">
            </div>

            <div class="navbar-end">

              <div class="navbar-item">
                if !user_ctx.is_authenticated() {
                    <div class="buttons">
                      <Link<Route> to={Route::SignIn} classes="button is-warning">
                        { "Sign In" }
                      </Link<Route>>
                    </div>
                  } else {
                    <div class="buttons">
                      <a {onclick} class="button is-warning">
                        { "Sign Out" }
                      </a>
                    </div>
                  }
              </div>

            </div>
          </div>
        </nav>
    }
}
