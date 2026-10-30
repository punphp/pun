<?php

namespace App;

final class WithVar
{
    private function bar(): ?\stdClass
    {
        return null;
    }

    public function run()
    {
        /** @var \stdClass|null $value */
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function positive()
    {
        $value = $this->bar();
        if ($value instanceof \stdClass) {
            return $value;
        }

        return null;
    }

    public function withDescription()
    {
        /** @var \stdClass|null $value the value */
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function otherComment()
    {
        // first
        /** @var \stdClass|null */
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }
}
-----
<?php

namespace App;

final class WithVar
{
    private function bar(): ?\stdClass
    {
        return null;
    }

    public function run()
    {
        return $this->bar();
    }

    public function positive()
    {
        return $this->bar();
    }

    public function withDescription()
    {
        /** @var \stdClass|null $value the value */
        return $this->bar();
    }

    public function otherComment()
    {
        // first
        return $this->bar();
    }
}
