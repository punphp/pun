<?php

namespace App;

use Symfony\Component\Validator\Constraints as Assert;

final class CastArrayArgument
{
    /**
     * @Assert\Length({"min"="5", "max"="x"})
     */
    public $value;
}
-----
<?php

namespace App;

use Symfony\Component\Validator\Constraints as Assert;

final class CastArrayArgument
{
    #[Assert\Length(['min' => 5, 'max' => 'x'])]
    public $value;
}
