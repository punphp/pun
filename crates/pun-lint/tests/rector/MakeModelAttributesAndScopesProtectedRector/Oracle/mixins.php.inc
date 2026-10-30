<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/** @mixin MixinB */
class MixinA {}

/** @mixin MixinA */
class MixinB {}

/** @mixin MixinA */
class CycleBase extends Model {}

/** @mixin Missing */
class MissingBase extends Model {}

class Cycle extends CycleBase
{
    public function scopeActive($query) {}
}

class FromMissing extends MissingBase
{
    public function scopeActive($query) {}
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/** @mixin MixinB */
class MixinA {}

/** @mixin MixinA */
class MixinB {}

/** @mixin MixinA */
class CycleBase extends Model {}

/** @mixin Missing */
class MissingBase extends Model {}

class Cycle extends CycleBase
{
    protected function scopeActive($query) {}
}

class FromMissing extends MissingBase
{
    protected function scopeActive($query) {}
}
