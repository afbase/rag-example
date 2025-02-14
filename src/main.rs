use gloo_console::log;
use gloo_net::http::Request;
use serde_json::{json, Value};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const CONTEXT: &str = r#"1. **What is the mission of /dev/color?**  
   /dev/color is dedicated to supporting and empowering Black technologists by fostering a strong community, providing career development resources, and advocating for diversity in the tech industry. The organization aims to help Black software engineers, founders, and leaders navigate challenges and advance their careers. Through mentorship, programs, and industry collaborations, /dev/color strives to create lasting change in the tech sector.

2. **What key achievements did /dev/color members accomplish in 2023?**  
   In 2023, /dev/color members celebrated numerous personal and professional milestones, including promotions, new job opportunities, and high-profile appearances. Many members reported feeling a stronger sense of community, professional support, and increased access to opportunities. The organization played a crucial role in fostering these successes through networking, mentorship, and career development initiatives.

3. **What is the A* Program, and how does it support Black technologists?**  
   The A* Program is /dev/color’s flagship initiative, designed to help Black software engineers and managers set and achieve ambitious career goals through peer support and mentorship. Participants engage in small, year-round squads where they collaborate on career development and problem-solving. The program fosters professional growth by offering networking, skill-building, and leadership development opportunities.

4. **What impact has the Executive Accelerator Program had on Black leaders in tech?**  
   Launched in 2023, the Executive Accelerator Program supports rising Black executives by providing mentorship, coaching, and executive training. Participants engaged in a mix of virtual and in-person sessions, gaining skills in leadership, networking, and board service preparation. As a result, 100% of participants reported making progress toward their career goals, with many feeling more confident and empowered in their leadership roles.

5. **What types of events and networking opportunities does /dev/color provide?**  
   /dev/color hosted six in-person events across major cities, including San Francisco, New York, Atlanta, and Seattle, with support from corporate partners like LinkedIn and Intuit. These events provided opportunities for Black technologists to connect, learn, and grow in a supportive environment. The organization also facilitated networking through online communities, peer mentorship, and industry collaborations.

6. **How does /dev/color collaborate with other organizations and tech companies?**  
   In 2023, /dev/color partnered with organizations like Black Product Managers Network, ColorStack, and Goodie Nation to expand its reach and impact. The organization also collaborated with corporate partners such as Pinterest, Grammarly, and Concrete Rose Capital to host panels, networking events, and professional development sessions. These partnerships helped bridge the gap between Black technologists and industry opportunities.

7. **What are the main sources of funding for /dev/color?**  
   /dev/color's primary funding comes from corporate partners, which contributed approximately $1.74 million in 2023. Additional revenue streams include individual contributions ($139,057) and program-related income ($93,345). These funds support the organization’s operations, programs, and community initiatives.

8. **What are some key statistics on member engagement and impact?**  
   In 2023, /dev/color had 760 active members across 386 companies and 39 A* squads. Surveys showed that 84% of members felt a deeper sense of community, 81% received direct support in their roles, and 68% increased their compensation. These figures highlight the tangible benefits of participation in /dev/color’s programs.

9. **Which corporate partners supported /dev/color in 2023?**  
   Some of /dev/color’s key corporate partners in 2023 included Atlassian, the Chan Zuckerberg Initiative, Etsy, GitLab, Grammarly, Intuit, LinkedIn, Pinterest, and the San Francisco Federal Reserve Bank. These organizations provided financial support, event sponsorships, and networking opportunities for members. Their contributions were essential in driving /dev/color’s mission forward.

10. **How can individuals or companies contribute to /dev/color’s mission?**  
   Individuals can support /dev/color by making financial contributions, participating in mentorship programs, and advocating for diversity in tech. Companies can collaborate by sponsoring events, providing funding, or offering professional development opportunities for Black technologists. These contributions help sustain the organization’s efforts to create a more inclusive and equitable tech industry."#;

#[function_component(App)]
fn app() -> Html {
    let question = use_state(String::new);
    let answer = use_state(String::new);
    let loading = use_state(|| false);

    let onsubmit = {
        let question = question.clone();
        let answer = answer.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let question_value = (*question).clone();
            let answer = answer.clone();
            let loading = loading.clone();

            if !question_value.is_empty() {
                loading.set(true);
                
                // Create the prompt with context
                let prompt = format!(
                    "You are an AI assistant helping answer questions about /dev/color. Use the following context to answer questions.  If you are highly confident in your answer based on the context provided, please cite the text from the context which provides the basis of your answer. \n\nContext:\n{}\n\nQuestion: {}\n\nProvide a succinct answer in plain language",
                    CONTEXT, question_value
                );

                spawn_local(async move {
                    // Make request to local Ollama instance
                    let response = Request::post("http://localhost:11434/api/generate")
                        .json(&json!({
                            "model": "llama3.3",
                            "prompt": prompt,
                            "stream": false
                        }))
                        .unwrap()
                        .send()
                        .await;

                    match response {
                        Ok(resp) => {
                            if let Ok(json) = resp.json::<Value>().await {
                                if let Some(response_text) = json["response"].as_str() {
                                    answer.set(response_text.to_string());
                                }
                            }
                        }
                        Err(err) => {
                            log!("Error:", err.to_string());
                            answer.set("Error communicating with Ollama".to_string());
                        }
                    }
                    loading.set(false);
                });
            }
        })
    };

    let oninput = {
        let question = question.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            question.set(input.value());
        })
    };

    html! {
        <div class="container mx-auto p-4 max-w-2xl">
            <h1 class="text-2xl font-bold mb-4">{"Ask about /dev/color"}</h1>
            
            <form {onsubmit} class="mb-4">
                <div class="flex gap-2">
                    <input
                        type="text"
                        placeholder="Ask a question..."
                        class="flex-grow p-2 border rounded"
                        value={(*question).clone()}
                        oninput={oninput}
                    />
                    <button
                        type="submit"
                        disabled={*loading}
                        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400"
                    >
                        {if *loading { "Loading..." } else { "Ask" }}
                    </button>
                </div>
            </form>

            if !(*answer).is_empty() {
                <div class="mt-4 p-4 bg-gray-100 rounded">
                    <h2 class="font-bold mb-2">{"Answer:"}</h2>
                    <p>{(*answer).clone()}</p>
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
