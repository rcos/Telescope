# Changelog
This changelog was created after version 0.6.4 was released. As such, information
version 0.6.4 and earlier may not be entirely accurate or complete. A best effort 
has been made to fill in the gaps. If you find an issue anywhere in this changelog, 
please submit a pull request fixing it. 

## Unreleased
- Make the `/whois` discord command ephemeral -- only the user who invokes the
  interaction will see the response.

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
- Fixed bug that prevented editing meetings from previous semesters. 
  ([#142])

## 0.6.0 - July 9th, 2021
- Added support for creating meetings. ([#96])
- Added support for editing meetings. ([#96])
- Changed appearance of footer. 
- Added rust documentation for developers to site. 
- Added system administrator user role. 
  ([#115])
- Fixed several links to open in new tabs. 
- Added semesters table in admin panel. 
- Added semester creation form in admin panel.
- Added semester edit form in admin panel. 
- Updated rust version to 1.53.0.

## 0.5.2 - May 26th, 2021
- Updated handlebars engine to 4.0.0. ([#113])
- Updated templates to work with handlebars 4.0.0.

## 0.5.1 - May 25th, 2021
- Fixed bug that caused internal server error when visiting a user's profile.
  ([#112])

## 0.5.0 - May 25th, 2021
- Created meeting details page.
- Added support for meetings marked as draft.
- Added link to handbook in navbar.
- Added support for linking RPI CAS account. [#5]
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
[#16]: https://github.com/rcos/Telescope/issues/16
[#96]: https://github.com/rcos/Telescope/issues/96
[#100]: https://github.com/rcos/Telescope/issues/100
[#112]: https://github.com/rcos/Telescope/issues/112
[#113]: https://github.com/rcos/Telescope/pull/113
[#114]: https://github.com/rcos/Telescope/issues/114
[#115]: https://github.com/rcos/Telescope/issues/115
[#142]: https://github.com/rcos/Telescope/issues/142
[#151]: https://github.com/rcos/Telescope/issues/151
