<?php

namespace Rector\Tests\CodeQuality\Rector\Expression\InlineIfToExplicitIfRector\Oracle;

final class MixedChain
{
    public function run($value)
    {
        $this->isReady() && $value->isReady() && $this->start();
        $this->isReady() && is_ready($value) && $this->start();
        $this->isReady() && $this->{$value}() && $this->start();
    }
}
-----
<?php

namespace Rector\Tests\CodeQuality\Rector\Expression\InlineIfToExplicitIfRector\Oracle;

final class MixedChain
{
    public function run($value)
    {
        if ($this->isReady() && $value->isReady()) {
            $this->start();
        }
        if ($this->isReady() && is_ready($value)) {
            $this->start();
        }
        if ($this->isReady() && $this->{$value}()) {
            $this->start();
        }
    }
}
