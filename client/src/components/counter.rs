use bounce::{use_atom, use_atom_value};
use crate::controllers::count::Count;
use yew::prelude::*;


#[function_component]
pub fn Counter() -> Html {
    let count_getter = use_atom_value::<Count>();
    let count_setter = use_atom::<Count>();

    let onclick = {
        let counter = count_setter.clone();
        move |_| {
            counter.set(Count {
                value: counter.value + 1,
            });
        }
    };

    html! {
        <div class="py-8 px-8 max-w-lg mx-auto bg-white rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6">
            <img class="block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0" src="/static/images/rustacean-flat-happy.png" alt="Ferris the Crab" />
            <div class="text-center space-y-2 sm:text-left">
                <div class="space-y-0.5">
                    <p class="text-lg text-black font-semibold">
                        {"Welcome to Rust Yew with Tailwind CSS!"}
                    </p>
                    <p class="text-slate-500 font-medium">
                        {"by Dylan Gonzales"}
                    </p>
                </div>
                <div class="flex flex-row">
                    <div class="basis-1/2">
                        <button
                            class="px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2"
                            {onclick}
                        >
                            {"Click Me!"}
                        </button>
                    </div>
                    <div class="basis-1/2">
                        <p class="text-black font-medium">
                            {"Count: "}{ &count_getter.value }
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
