<?php

// The Symfony classes the fixtures use, which Rector's own test run loads from its vendor/.

namespace Symfony\Component\HttpFoundation;

class Response
{
}

class JsonResponse extends Response
{
}

namespace Symfony\Bundle\FrameworkBundle\Controller;

abstract class AbstractController
{
}

abstract class Controller
{
}
