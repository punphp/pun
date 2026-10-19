<?php

namespace App;

final class Cases
{
    /**
     * @var
     */
    private \DateTime $noType;

    /**
     * @var \DateTimeInterface
     */
    private \DateTime $hooked {
        get => $this->hooked;
    }
}
-----
<?php

namespace App;

final class Cases
{
    /**
     * @var
     */
    private \DateTime $noType;

    private \DateTimeInterface $hooked {
        get => $this->hooked;
    }
}
