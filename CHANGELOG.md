# Changelog
This changelog was created after version 0.6.4 was released. As such, information
version 0.6.4 and earlier may not be entirely accurate or complete. A best effort 
has been made to fill in the gaps. If you find an issue anywhere in this changelog, 
please submit a pull request fixing it. 

## Unreleased
- Added meeting counts and filtering by semester in admin panel. 

## 0.8.5 - December 31st, 2021
- Changes to the config file:
  - The public telescope URL is now in the root of the config rather than the Discord section.  
- Dynamic OGP tags. ([#218])
- Server side cohort validation. ([#220])

## 0.8.4 - December 29th, 2021
- Disabled signup using anything except Discord. ([#217])
- Added descriptions OGP tag. ([#216]) 

## 0.8.3 - December 22nd, 2021
- Updated minimum rust version to 1.57.0. 
- Added [Open Graph Protocol](https://ogp.me/) meta tags to site. ([#205])
- Fixed critical security issues reported by [Chris Reed](https://github.com/cjreed121) (Thanks, Chris, for reporting these to me): 
  - XSS bug in the meeting description text box. 
  - Privilege escalation bug on the user settings page. 
- Fixed two typos also reported by [Chris Reed](https://github.com/cjreed121) (Thanks again Chris)
- Updates to a variety of dependencies. 

## 0.8.2 - November 30th, 2021
- Fixed bug that prevented user deletion. ([#204])

## 0.8.1 - November 19th, 2021
- Fixed bug that prevented meeting creation. 

## 0.8.0 - November 17th, 2021
- Replaced all usernames with user IDs. ([#130])
- Updated Hasura to v2.0.10 

## 0.7.3 - November 3rd, 2021
- Changes to the config file: 
  - Now only a single RCOS Discord server ID is accepted instead of a list of IDs.
- Fixed error caused by unlinking Discord account while authenticated with Discord. ([#185])
- Update Rust to 1.56 and Rust 2021 Edition. ([#186])
- Change RCOS contact email from rcos.management@gmail.com to rcos-leadership@googlegroups.com. ([#188])
- Add Discord gateway. ([#178])
- User deletion functionality. ([#15], [#189])

## 0.7.2 - October 12th, 2021
- Account linking with Discord ([#5], [#181])
- Updated Discord colors and icons ([#116], [#181], [#182]) 
- Tweaked format of Discord Guild IDs in config file ([#179])

## 0.7.1 - September 29th, 2021
- Make the `/whois` discord command ephemeral -- only the user who invokes the
  interaction will see the response.
- Fixed "Schedule" link on the home page to render correctly on Safari. ([#176])
- Updated everything to work at <https://rcos.io> instead of <https://telescope.rcos.io>. 

## 0.7.0 - September 9th, 2021
- Name change functionality. ([#16])
- Cohort edit functionality. ([#16])
- Show draft meetings on profile pages.
- Fixed homepage link to include past developers.

## 0.6.5 - August 31st, 2021
- Created changelog. ([#151])
- Updated RPI CAS URL. ([#114])

## 0.6.4 - July 14th, 2021
- Fixed bug that prevented external presentation urls from being appropriately shown 
  in the meeting edit form. 

## 0.6.3 - July 14th, 2021
- Added support for deleting meetings. ([#96])

## 0.6.2 - July 13th, 2021
- Fixed access bug that prevented users from viewing drafts of meetings that they host.

## 0.6.1 - July 10th, 2021
- Fixed bug that prevented editing meetings from previous semesters. ([#142])

## 0.6.0 - July 9th, 2021
- Added support for creating meetings. ([#96])
- Added support for editing meetings. ([#96])
- Changed appearance of footer. 
- Added rust documentation for developers to site. 
- Added system administrator user role. ([#115])
- Fixed several links to open in new tabs. 
- Added semesters table in admin panel. 
- Added semester creation form in admin panel.
- Added semester edit form in admin panel. 
- Updated rust version to 1.53.0.

## 0.5.2 - May 26th, 2021
- Updated handlebars engine to 4.0.0. ([#113])
- Updated templates to work with handlebars 4.0.0.

## 0.5.1 - May 25th, 2021
- Fixed bug that caused internal server error when visiting a user's profile. ([#112])

## 0.5.0 - May 25th, 2021
- Created meeting details page.
- Added support for meetings marked as draft.
- Added link to handbook in navbar.
- Added support for linking RPI CAS account. ([#5])
- Created Discord bot. ([#100])
    - Added `/whois` command.
- Removed email client and templates. 
- Fixed several bugs relating to timezones.

## 0.4.2 - March 25th, 2021
- Updated rust version to 1.51.0.
- Added meeting type indicators to meetings page. 
- Centered text on links in the meetings page.

## 0.4.1 - March 25th, 2021
- Tweaked style and appearance of meetings page.
- Fixed bug that incremented meeting page date query on refresh.

## 0.4.0 - March 25th, 2021
- Created meetings page.
- Tweaked URL formatting for user profiles.

## 0.3.1 - March 23rd, 2021
- No release notes available.

## 0.3.0 - March 23rd, 2021
- No release notes available.

## 0.2.5 - March 8th, 2021
- No release notes available.

## 0.2.4 - March 8th, 2021
- No release notes available.

## 0.2.3 - March 8th, 2021
- No release notes available.

## 0.2.2 - March 8th, 2021
- No release notes available.

## 0.2.1 - March 8th, 2021
- No release notes available.

## Initial release: 0.2.0 - March 8th, 2021
- No release notes available.

<!-- links -->
[#5]: https://github.com/rcos/Telescope/issues/5
[#15]: https://github.com/rcos/Telescope/issues/15
[#16]: https://github.com/rcos/Telescope/issues/16
[#96]: https://github.com/rcos/Telescope/issues/96
[#100]: https://github.com/rcos/Telescope/issues/100
[#112]: https://github.com/rcos/Telescope/issues/112
[#113]: https://github.com/rcos/Telescope/pull/113
[#114]: https://github.com/rcos/Telescope/issues/114
[#115]: https://github.com/rcos/Telescope/issues/115
[#116]: https://github.com/rcos/Telescope/issues/116
[#130]: https://github.com/rcos/Telescope/issues/130
[#142]: https://github.com/rcos/Telescope/issues/142
[#151]: https://github.com/rcos/Telescope/issues/151
[#176]: https://github.com/rcos/Telescope/pull/176
[#178]: https://github.com/rcos/Telescope/issues/178
[#179]: https://github.com/rcos/Telescope/pull/179
[#181]: https://github.com/rcos/Telescope/pull/181
[#182]: https://github.com/rcos/Telescope/pull/182
[#185]: https://github.com/rcos/Telescope/issues/185
[#186]: https://github.com/rcos/Telescope/pull/186
[#188]: https://github.com/rcos/Telescope/issues/188
[#189]: https://github.com/rcos/Telescope/pull/189
[#204]: https://github.com/rcos/Telescope/pull/204
[#205]: https://github.com/rcos/Telescope/pull/205
[#216]: https://github.com/rcos/Telescope/pull/216
[#217]: https://github.com/rcos/Telescope/pull/217
[#218]: https://github.com/rcos/Telescope/pull/218
[#220]: https://github.com/rcos/Telescope/pull/220
