<?php

namespace Tests\Feature {
    uses(\Foo\Bar::class);
    test('a', function () {});
}
-----
<?php

namespace Tests\Feature {
    test('a', function () {});
}
