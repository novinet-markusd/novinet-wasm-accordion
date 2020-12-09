# novinet-wasm-accordion

A repository for an accordion in web assembly written in rust.

## Build

to build the project [rust](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) are required on your machine.

use `wasm-pack build --release --target web` to build the project.

## Usage

create a folder on your webserver and insert the following files (minimum):

- pkg/novinet_wasm_accordion.js
- pkg/novinet_wasm_accordion_bg.js
- pkg/novinet_wasm_accordion_bg.wasm

add this script tag to your html:

```javascript
<script type="module">
import init, {add_event_listeners_accordion} 
from 'url/to/folder/novinet_wasm_accordion.js';

async function run() {
    await init();
    add_event_listeners_accordion();
}
run();
</script>
```
