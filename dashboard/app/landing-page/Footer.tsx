import {
  Footer,
  FooterCopyright,
  FooterDivider,
  FooterIcon,
  FooterLink,
  FooterLinkGroup,
  FooterTitle,
} from 'flowbite-react';
import { BsGithub } from 'react-icons/bs';
import AppLogo from '../ui/app-logo';

const FooterComp = () => {
  return (
    <Footer container className="bg-white dark:bg-gray-900">
      <div className="container mx-auto">
        <div className="grid w-full justify-between sm:flex sm:justify-between md:flex md:grid-cols-1">
          <div className="hidden w-full lg:flex">
            <AppLogo w={400} h={312} />
          </div>
          <div className="grid grid-cols-2 gap-8 sm:mt-4 sm:grid-cols-3 sm:gap-6">
            <div>
              <FooterTitle title="about" />
              <FooterLinkGroup col>
                <FooterLink href="#">About</FooterLink>
                <FooterLink href="#">Library</FooterLink>
                <FooterLink href="#">Documentation</FooterLink>
              </FooterLinkGroup>
            </div>
            <div>
              <FooterTitle title="Follow us" />
              <FooterLinkGroup col>
                <FooterLink
                  href="https://github.com/oele-isis-vanderbilt/SyncFlow.git"
                  target="_blank"
                >
                  Github
                </FooterLink>
              </FooterLinkGroup>
            </div>

            <div>
              <FooterTitle title="Contact" />
              <FooterLinkGroup col>
                <FooterLink href="#">Contact Us</FooterLink>
                <FooterLink href="#">Terms &amp; Conditions</FooterLink>
              </FooterLinkGroup>
            </div>
          </div>
        </div>
        <FooterDivider className="m-4" />
        <div className="w-full sm:flex sm:items-center sm:justify-between">
          <FooterCopyright
            href="https://wp0.vanderbilt.edu/oele/"
            by="OELE™"
            year={2024}
          />
          <div className="mt-4 flex space-x-6 sm:mt-0 sm:justify-center">
            <FooterIcon
              href="https://github.com/oele-isis-vanderbilt"
              icon={BsGithub}
              target="_blank"
            />
          </div>
        </div>
      </div>
    </Footer>
  );
};

export default FooterComp;
