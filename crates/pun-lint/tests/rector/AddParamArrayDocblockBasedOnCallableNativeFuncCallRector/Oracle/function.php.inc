<?php

function run_items(array $items): void
{
    usort($items, fn (\DateTime $a, \DateTime $b) => $a <=> $b);
}
-----
<?php

/**
 * @param \DateTime[] $items
 */
function run_items(array $items): void
{
    usort($items, fn (\DateTime $a, \DateTime $b) => $a <=> $b);
}
