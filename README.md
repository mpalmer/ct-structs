This repository contains the `ct-structs` Rust crate, which represents the data structures used by the [Certificate Transparency](https://certificate.transparency.dev/) protocol, as specified by [RFC6962](https://datatracker.ietf.org/doc/html/rfc6962).

There are not a lot of docs, mostly because the data structures are as-close-to-exact representations of the RFC structures, and I'm presuming that if you're using this crate, you're reasonably familiar with Certificate Transparency already.

If you're looking for any sort of "complete" implementation of a CT client, monitor, user agent library, or any number of other things, this is not the right place.


# Licence

Unless otherwise stated, everything in this repo is covered by the following
copyright notice:

    Copyright (C) 2023  Matt Palmer <matt@hezmatt.org>

    This program is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License version 3, as
    published by the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

