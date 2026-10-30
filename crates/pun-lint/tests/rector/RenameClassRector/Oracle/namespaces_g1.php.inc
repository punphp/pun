<?php

namespace App {
    use Foo\Unused;
}

namespace B {
    use Foo\Unused2;
}
-----
<?php

namespace App;


namespace B;

