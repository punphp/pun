<?php

namespace Vendor\PrivateApi {
    /** @internal */
    function hidden($value = null): void {}
    function publicApi(): void {}
    hidden();
    hidden(...);
}

namespace Vendor\Other {
    use function Vendor\PrivateApi\hidden as secret;
    secret();
    secret(...);
    'input' |> secret(...);
}

namespace VendorExtra {
    use function Vendor\PrivateApi\hidden as secret;
    secret();
    secret(...);
    'input' |>
        secret(...);
    \Vendor\PrivateApi\publicApi();
    \Vendor\PrivateApi\publicApi(...);
    \strlen('input');
    \strlen(...);
    missing();
    missing(...);
    $dynamic = 'Vendor\\PrivateApi\\hidden';
    $dynamic();
    $dynamic(...);
    'input' |> $dynamic;
}

namespace {
    /** @internal */
    function globalHidden(): void {}
    globalHidden();
    globalHidden(...);
    class globalHidden {
        function allowed(): void {
            globalHidden();
            globalHidden(...);
        }
    }
}

namespace Other {
    \globalHidden();
    \globalHidden(...);
}

namespace vendor {
    \Vendor\PrivateApi\hidden();
    \Vendor\PrivateApi\hidden(...);
}
