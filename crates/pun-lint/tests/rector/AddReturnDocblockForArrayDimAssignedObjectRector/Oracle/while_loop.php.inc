<?php

final class Row {}

final class SomeClass
{
    public function run(int $n): array
    {
        $rows = [];
        while ($n-- > 0) {
            if ($n > 1) {
                $rows[] = new Row();
            }
        }

        return $rows;
    }
}
-----
<?php

final class Row {}

final class SomeClass
{
    /**
     * @return \Row[]
     */
    public function run(int $n): array
    {
        $rows = [];
        while ($n-- > 0) {
            if ($n > 1) {
                $rows[] = new Row();
            }
        }

        return $rows;
    }
}
