<?php

namespace App;

class Maybe
{
    public function maybe(): ?static
    {
        return null;
    }

    public function either(): static|int
    {
        return 1;
    }

    public function viaMaybe()
    {
        return $this->maybe();
    }

    public function viaEither()
    {
        return $this->either();
    }
}
-----
<?php

namespace App;

class Maybe
{
    public function maybe(): ?static
    {
        return null;
    }

    public function either(): static|int
    {
        return 1;
    }

    public function viaMaybe(): ?self
    {
        return $this->maybe();
    }

    public function viaEither(): self|int
    {
        return $this->either();
    }
}
