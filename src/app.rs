use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <script>
            window.addEventListener("contextmenu", (e) => {
                const allowedTypes = ["INPUT", "TEXTAREA"];
                if (!allowedTypes.includes(e.target.nodeName)) {
                    e.preventDefault();
                }
            });
        </script>
        <main class="container">
        </main>
    }
}
