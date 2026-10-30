<?php

namespace App;

final class More
{
    private function bar(): ?\stdClass
    {
        return null;
    }

    public function plainDoc()
    {
        /** hello */
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function docThenComment()
    {
        /** @var \stdClass|null $value */
        // after
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function blockComment()
    {
        /* block */
        $value = $this->bar();
        if (! $value instanceof \stdClass) {
            return null;
        }

        return $value;
    }

    public function otherVar()
    {
        /** @var int $other */
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

final class More
{
    private function bar(): ?\stdClass
    {
        return null;
    }

    public function plainDoc()
    {
        /** hello */
        return $this->bar();
    }

    public function docThenComment()
    {
        // after
        return $this->bar();
    }

    public function blockComment()
    {
        /* block */
        return $this->bar();
    }

    public function otherVar()
    {
        return $this->bar();
    }
}
