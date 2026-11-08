<?php

final class SingleLineDoc
{
    /** @deprecated */
    public function process(array $data): void
    {
        echo $data[1];
    }
}
-----
<?php

final class SingleLineDoc
{
    /**
     * @deprecated
     * @param array<int, mixed> $data
     */
    public function process(array $data): void
    {
        echo $data[1];
    }
}
