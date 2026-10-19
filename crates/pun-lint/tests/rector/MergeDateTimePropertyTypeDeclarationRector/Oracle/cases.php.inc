<?php

namespace App;

use DateTimeInterface as Moment;

final class Event
{
    /**
     * @var Moment
     */
    private \DateTime $start;

    /**
     * Some text.
     *
     * @var \DateTimeInterface $end
     */
    private \DateTime $end;

    /**
     * @var \DateTimeInterface|null
     */
    private ?\DateTime $maybe;

    /**
     * @var \DateTimeInterface
     * @var \DateTimeInterface
     */
    private \DateTime $twice;

    /**
     * @var \DateTimeInterface
     */
    private \datetime $lower;

    /**
     * @return void
     */
    private \DateTime $noVar;

    /** @var \DateTimeInterface */
    private \DateTime $oneLine;

    /**
     * @var \DateTimeInterface
     */
    private string $other;
}

trait Times
{
    /**
     * @var \DateTimeInterface
     */
    private \DateTime $start;
}
-----
<?php

namespace App;

use DateTimeInterface as Moment;

final class Event
{
    private \DateTimeInterface $start;

    /**
     * Some text.
     */
    private \DateTimeInterface $end;

    /**
     * @var \DateTimeInterface|null
     */
    private ?\DateTime $maybe;

    private \DateTimeInterface $twice;

    /**
     * @var \DateTimeInterface
     */
    private \datetime $lower;

    /**
     * @return void
     */
    private \DateTime $noVar;

    private \DateTimeInterface $oneLine;

    /**
     * @var \DateTimeInterface
     */
    private string $other;
}

trait Times
{
    /**
     * @var \DateTimeInterface
     */
    private \DateTime $start;
}
