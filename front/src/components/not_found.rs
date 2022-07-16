use yew::{function_component, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {
    <div class="section container">
      <p class="block">
        { "Not found! "}
        <a href="/">
          { "Go back"}
        </a>
        { "." }
      </p>
    </div>
  }
}
