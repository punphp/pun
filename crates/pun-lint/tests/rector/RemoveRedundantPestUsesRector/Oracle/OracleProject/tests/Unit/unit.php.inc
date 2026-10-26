<?php

uses(\Tests\Fixture\SampleTrait::class, \Other\Trait1::class, \Illuminate\Foundation\Testing\RefreshDatabase::class);
-----
<?php

uses(\Illuminate\Foundation\Testing\RefreshDatabase::class);
