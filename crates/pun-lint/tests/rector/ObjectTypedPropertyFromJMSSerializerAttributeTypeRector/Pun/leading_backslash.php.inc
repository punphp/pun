<?php

namespace App;

use JMS\Serializer\Annotation\Type;

// Rector writes `private ?\\DateTimeImmutable $leading = null;` (a second `\` before the written one), which does not
// parse; Pun leaves the property.
final class LeadingBackslash
{
    #[Type('\DateTimeImmutable')]
    private $leading;
}
