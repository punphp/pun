<?php

namespace Tests\Feature {
    use Illuminate\Foundation\Testing\RefreshDatabase;

    it('a', fn () => 1);
    uses(RefreshDatabase::class);
}

namespace {
    uses(Illuminate\Foundation\Testing\RefreshDatabase::class, Keep::class);
}
-----
<?php

namespace Tests\Feature {
    use Illuminate\Foundation\Testing\RefreshDatabase;

    it('a', fn () => 1);
}

namespace {
    uses(Keep::class);
}
