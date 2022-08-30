.. image:: scieldas.png
    :alt: scieldas sigil
    :align: center

scieldas
  noun: shields, protection

Scieldas-rs is an ongoing rust port of `Scieldas`_ my metadata badges for
open source project READMEs, inspired by `Shields.io`_.

Built with Rocket and Nix.

Running Scieldas
----------------

Binary
......

To build and run the Scieldas binary::

    $ nix build
    $ ./result/bin/scieldas

Docker Image
............

To build and run the Scieldas docker image::

    $ nix build .#docker
    $ docker load < result
    $ docker run -p 8000:8000 scieldas:<tag>

.. _Scieldas: https://github.com/autophagy/scieldas
.. _Shields.io: https://shields.io
