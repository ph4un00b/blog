use crate::{components::icons::StrToIcon, models::article::Article};
use leptos::*;
use leptos_markdown::Markdown;

#[component]
pub fn BlogContent(#[prop()] article: Article) -> impl IntoView {
    view! {
        <div class="group flex flex-col gap-y-6 border border-black p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between">
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left">
                {article.title}
            </h1>
            <div class="flex flex-col">
                <div class="flex flex-row gap-4 text-sm items-center">
                    <h5>{article.author}</h5>
                    <hr class="h-[0.875rem] w-px bg-gray-700 border-0"/>
                    <div class="flex flex-row gap-2 items-center">
                        {article
                            .social
                            .iter()
                            .map(|(net, url)| {
                                view! {
                                    <a target="_blank" href=url>
                                        <StrToIcon v=net.to_string() size=15/>
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
                <span class="text-gray-400 text-sm items-center">{article.date}</span>
            </div>
            <Markdown src=article.content.to_string()/>
        </div>
    }
}
