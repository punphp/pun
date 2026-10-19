<?php

namespace App;

use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;

class SkipAbstractControllerResponse extends AbstractController
{
    public function create()
    {
        return new JsonResponse();
    }
}
