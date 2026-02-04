use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn InputForm() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (factors, set_factors) = signal(vec![1]);
    let (primes, set_primes) = signal(vec![0]);

    view! {
            <input type="text"
                // adding :target gives us typed access to the element
                // that is the target of the event that fires
                on:input:target=move |ev| {
                    let num_as_str = ev.target().value();
                    let num: i32 = num_as_str.parse::<i32>().unwrap_or(0);
                    // .value() returns the current value of an HTML input element
                    set_count.set(num);
                }

                // the `prop:` syntax lets you update a DOM property,
                // rather than an attribute.
                prop:value=count
            />
    <button on:click=move |_| {
                set_factors.set(count.get().factors());
                set_primes.set(count.get().primes());
            }>

                "Factors: " {move || factors.get().iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ")}
            </button>
    <button on:click=move |_| {
                set_factors.set(count.get().factors());
                set_primes.set(count.get().primes())
            }>

                "Primes: " {move || primes.get().iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ")}
            </button>
        }
}

trait GetPrimes<T>
where
    T: GetFactors<T>,
{
    fn primes(&self) -> Vec<T>;
}

impl GetPrimes<i32> for i32 {
    fn primes(&self) -> Vec<i32> {
        let mut primes: Vec<i32> = vec![];
        for i in 1..self + 1 {
            let mut factors: Vec<i32> = i.factors();
            if factors.len() == 2 {
                primes.push(i)
            }
        }
        primes
    }
}

trait GetFactors<T> {
    fn factors(&self) -> Vec<T>;
}

impl GetFactors<i32> for i32 {
    fn factors(&self) -> Vec<i32> {
        let stop = self + 1; // add 1 just to be safe
        let mut factors: Vec<i32> = vec![];
        for i in 1..stop {
            if self % i == 0 {
                factors.push(i);
            }
        }
        factors
    }
}

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| {
            set_count.set(count.get() + increment)
        }>

            "Click me: " {count}
        </button>
    }
}
