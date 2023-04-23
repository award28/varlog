use yew::prelude::*;

#[hook]
pub fn use_input_state<T: 'static>(default: T) -> (UseStateHandle<T>, Callback<T>) {
    let handle = use_state(|| default);

    let on_change = {
        let handle = handle.clone();

        Callback::from(move |value| {
            handle.set(value);
        })
    };
    (handle, on_change)
}
