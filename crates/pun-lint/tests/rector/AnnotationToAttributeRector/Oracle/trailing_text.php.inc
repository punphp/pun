<?php

namespace App;

use Symfony\Component\Routing\Annotation\Route;
use Doctrine\ORM\Mapping as ORM;

final class TrailingText
{
    /**
     * @Route("/a") some text after
     * @ORM\Column(nullable="false") )
     */
    public function action()
    {
    }
}
-----
<?php

namespace App;

use Symfony\Component\Routing\Annotation\Route;
use Doctrine\ORM\Mapping as ORM;

final class TrailingText
{
    /**
     * some text after
     */
    #[Route(path: '/a')]
    #[ORM\Column(nullable: false)]
    public function action()
    {
    }
}
