use leptos::prelude::*;

#[component]
pub fn InputForm() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (factors, set_factors) = signal(vec![1]);
    let (primes, set_primes) = signal(vec![0]);

    view! {
            <input type="text"
                on:input:target=move |ev| {
                    let num_as_str = ev.target().value();
                    let num: i32 = num_as_str.parse::<i32>().unwrap_or(0);
                    set_count.set(num);
                }

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
    fn primes_in_range(&self, min: i32) -> Vec<i32>;
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
fn primes_in_range(&self, min: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![];
    for i in min..self+1 {
        if i.factors().len() == 2 {
            primes.push(i);
    }
        primes
    }
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
