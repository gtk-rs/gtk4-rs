# Virtual methods

Example for how to create a base widget which exposes virtual methods that other widgets can override.
In this case, a `BaseButton` class is defined which is a gtk::Button with two additional methods:

* `sync_method`: Synchronous method that updates the button label to `BaseButton sync`, or `BaseButton sync {extra_text}`. This showcases how arguments
   can be passed to virtual methods. The `Option<String>` has to be boxed to be FFI-safe.
* `async_method`: Asynchronous method that updates the button label to `BaseButton async`

`DerivedButton` overrides the two methods. During construction `sync_method` is called in
`BaseButton` which will set the label to `BaseButton sync` for `BaseButton` and to
`DerivedButton sync` for `DerivedButton`.
