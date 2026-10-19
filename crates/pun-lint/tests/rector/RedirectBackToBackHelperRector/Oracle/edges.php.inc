<?php
namespace {
    use Illuminate\Support\Facades\{Redirect, function helper, const VALUE};
    use function Other\another;
    Redirect::back(302);
    $class::back();
    Missing::back();
}
-----
<?php
namespace {
    use Illuminate\Support\Facades\{Redirect, function helper, const VALUE};
    use function Other\another;
    back(302);
    $class::back();
    Missing::back();
}
