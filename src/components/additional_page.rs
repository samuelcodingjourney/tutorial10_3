use yew::prelude::*;

#[function_component(AdditionalPage)]
pub fn additional_page() -> Html {
    html! {
        <div class="container mx-auto mt-10">
            <h1 class="text-3xl font-bold text-center mb-5">{"Welcome to the Additional Page!"}</h1>
            <p class="text-lg text-gray-800 text-center mb-5">{"Explore new horizons with YewChat!"}</p>
            <div class="grid grid-cols-2 gap-4">
                <div class="bg-blue-500 text-white p-5 rounded-lg">
                    <h2 class="text-xl font-bold mb-3">{"Discover New Friends"}</h2>
                    <p>{"Connect with people from all over the world and make new friends effortlessly."}</p>
                </div>
                <div class="bg-green-500 text-white p-5 rounded-lg">
                    <h2 class="text-xl font-bold mb-3">{"Express Yourself"}</h2>
                    <p>{"Share your thoughts, feelings, and ideas with others through creative messages and emojis."}</p>
                </div>
            </div>
            <p class="text-center mt-8">{"Ready to dive in? Start chatting now!"}</p>
        </div>
    }
}
