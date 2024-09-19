<script lang="ts">
    import { Icon, Navbar, NavbarBrand, NavbarToggler, Collapse, Nav, NavItem, NavLink, Dropdown, DropdownToggle, DropdownMenu, DropdownItem  } from '@sveltestrap/sveltestrap';

    let navbarIsOpen = false;
  </script>

  <Navbar color="light" light expand="md">
    <NavbarBrand href="/">Finances</NavbarBrand>
    <NavbarToggler on:click={() => (navbarIsOpen = !navbarIsOpen)} />
    <Collapse {navbarIsOpen} navbar expand="md">
      <Nav class="ms-auto" navbar>
        <NavItem>
          <NavLink href="accounts/">Accounts</NavLink>
        </NavItem>
        <Dropdown nav inNavbar>
          <DropdownToggle nav caret>Rentals</DropdownToggle>
          <DropdownMenu end>
            <DropdownItem href="rentals/lope-de-haro">Lope de Haro</DropdownItem>
            <DropdownItem href="rentals/garaje-molinos">Garaje Molinos</DropdownItem>
          </DropdownMenu>
        </Dropdown>
        <NavItem>
          <NavLink href="taxes/">Tax declaration</NavLink>
        </NavItem>
        <NavItem>
          <NavLink href="settings/"><Icon name="gear-fill"/></NavLink>
        </NavItem>
      </Nav>
    </Collapse>
  </Navbar>

<slot></slot>
