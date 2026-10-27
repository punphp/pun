<?php

class MixedFluentReturn
{
    public function resolve(bool $reuse)
    {
        if ($reuse) {
            return $this;
        }

        return new self();
    }
}
-----
<?php

class MixedFluentReturn
{
    public function resolve(bool $reuse): self
    {
        if ($reuse) {
            return $this;
        }

        return new self();
    }
}
