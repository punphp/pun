<?php

namespace App;

final class Item {}

final class Objects
{
    public function go(): void
    {
        $this->run([new Item(), new Item()]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

namespace App;

final class Item {}

final class Objects
{
    public function go(): void
    {
        $this->run([new Item(), new Item()]);
    }

    /**
     * @param \App\Item[] $items
     */
    private function run(array $items): void
    {
    }
}
