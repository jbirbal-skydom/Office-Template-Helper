<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/jbirbal-skydom/Office-Template-Helper">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center"> Office Template Helper</h3>

  <p align="center">
    project_description
    <br />
    <a href="https://github.com/jbirbal-skydom/Office-Template-Helper"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jbirbal-skydom/Office-Template-Helper">View Demo</a>
    ·
    <a href="https://github.com/jbirbal-skydom/Office-Template-Helper/issues">Report Bug</a>
    ·
    <a href="https://github.com/jbirbal-skydom/Office-Template-Helper/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Office Template Helper is a versatile desktop application designed to enhance the productivity of Microsoft Office users by enabling the easy integration of add-ons directly into Office files via XML modifications. With a user-friendly graphical interface, Office Template Helper allows users to seamlessly select and inject add-on templates, such as brainstorming aids and cross-functional flowcharts, into existing Office documents.

### Built With

* [![Slint][slint-lang]][Slint-url]
* [![Rust][rust-lang]][Rust-url]
* [![C][c-lang]][C-url]
* [![GCC][gcc-badge]][GCC-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

**Key Features:**

- **Intuitive GUI**: Offers a simple, clean, and easy-to-navigate interface that allows even non-technical users to modify Office files effortlessly.
- **Add-On Selection**: Users can choose from a list of available add-ons, including specialized templates for brainstorming sessions and detailed flowchart designs.
- **XML Modification**: Automatically adds the necessary XML line to an Office file to activate the selected add-on, simplifying what would otherwise be a manual and complex process.
- **Compatibility**: Supports a wide range of Office file formats, ensuring broad usability across different Office applications such as Visio, Word, and Excel.
- **Efficiency**: Streamlines workflow by integrating add-ons quickly, reducing the time and effort typically required for template setup and customization.


**Benefits:**

- **Enhanced Productivity**: Quickly add complex templates to documents, enabling faster preparation and more effective information organization.
- **Customization**: Offers the flexibility to customize documents with advanced tools, catering to specific project needs and preferences.
- **Ease of Use**: Removes the technical barriers associated with XML file manipulation, making advanced document customization accessible to all users.


**Intended Users:**

OfficeTemplateHelper is ideal for business professionals, project managers, and anyone involved in document preparation who seeks to enhance the functionality of their Office documents with advanced graphical templates and tools.

<p align="right">(<a href="#readme-top">back to top</a>)</p>





<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* npm
  ```sh
  npm install npm@latest -g
  ```

### Installation

1. Get a free API Key at [https://example.com](https://example.com)
2. Clone the repo
   ```sh
   git clone https://github.com/jbirbal-skydom/Office-Template-Helper.git
   ```
3. Install NPM packages
   ```sh
   npm install
   ```
4. Enter your API in `config.js`
   ```js
   const API_KEY = 'ENTER YOUR API';
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] GUI
- [ ] Microsoft Products
  - [ ] Visio
    - [X] Brainstorm
    - [ ] Cross-Functional Flowchart
    - [ ] Timeline
  - [ ] Excel
  - [ ] Words
  - [ ] PowerPoint
  - [ ] Project
  - [ ] Outlook


___ 
## Explanation:  

## Understanding the File Structure of Microsoft Office Files

Modern Microsoft Office files, such as those created by Office 2007 and later versions, are built using the Open XML standard. This design choice provides a robust, open, and reusable data format. Below, we explore how these files are structured, particularly focusing on their ZIP and XML components.

### ZIP Format Overview

Office files (.docx, .xlsx, .pptx, etc.) are essentially ZIP archives that contain multiple files and folders. These ZIP archives consolidate various parts of a document into a single, compressed file, making storage and transfer more efficient. When you rename an Office file to have a `.zip` extension and open it with a zip utility, you will see its internal structure.

### Internal XML Structure

Inside the ZIP archive, the content of the file is organized into a series of folders and XML files that describe the document's properties, content, and formatting:

- **`[Content_Types].xml`**: This file defines the types of files contained within the ZIP archive and their corresponding MIME types.

- **`_rels`**: A folder containing `.rels` files, which are relationship files mapping the connections between various parts of the document (like which images are used where).

- **`docProps`**: This folder contains XML files that store properties about the document itself, such as the author, creation date, and modification date (`core.xml`), as well as custom-defined properties (`custom.xml`) and application-specific properties (`app.xml`).

- **`word`, `excel`, `ppt`** (depending on the type of Office file): These directories contain the actual content of the document. For instance, in a Word file (`word`), you will find:
  - **`document.xml`**: The main document content.
  - **`styles.xml`**: Definitions of the styles used in the document.
  - **`settings.xml`**: Document-specific settings like spelling, grammar checks, and compatibility settings.
  - **`theme`**: Theme information used across the document.
  - **`media`**: A folder containing embedded media files such as images.

### Advantages of Using ZIP and XML

1. **Compression**: Using ZIP format allows the Office files to be compressed, reducing the file size and making it easier to share and store.
2. **Modularity and Accessibility**: The use of XML makes data extraction and manipulation straightforward, which can be particularly useful for data analysis and automated report generation.
3. **Standardization**: XML provides a standard format that can be read by many different types of software, enhancing interoperability.

The combination of ZIP and XML in Office files aligns with modern data handling practices by ensuring that documents are compact, robust, and ready for integration with various data systems.



___

See the [open issues](https://github.com/jbirbal-skydom/Office-Template-Helper/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the WTFPL License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Jason Birbal - officehelper@skydom.ai

Project Link: [https://github.com/jbirbal-skydom/Office-Template-Helper](https://github.com/jbirbal-skydom/Office-Template-Helper)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Zip File](https://superuser.com/questions/145479/excel-edit-the-xml-inside-an-xlsx-file)
* [vsdx python software ](https://github.com/dave-howard/vsdx/)
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/jbirbal-skydom/Office-Template-Helper.svg?style=for-the-badge
[contributors-url]: https://github.com/jbirbal-skydom/Office-Template-Helper/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jbirbal-skydom/Office-Template-Helper.svg?style=for-the-badge
[forks-url]: https://github.com/jbirbal-skydom/Office-Template-Helper/network/members
[stars-shield]: https://img.shields.io/github/stars/jbirbal-skydom/Office-Template-Helper.svg?style=for-the-badge
[stars-url]: https://github.com/jbirbal-skydom/Office-Template-Helper/stargazers
[issues-shield]: https://img.shields.io/github/issues/jbirbal-skydom/Office-Template-Helper.svg?style=for-the-badge
[issues-url]: https://github.com/jbirbal-skydom/Office-Template-Helper/issues
[license-shield]: https://img.shields.io/github/license/jbirbal-skydom/Office-Template-Helper.svg?style=for-the-badge
[license-url]: https://github.com/jbirbal-skydom/Office-Template-Helper/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png


 <!-- Badge  -->

[rust-lang]: https://img.shields.io/badge/Rust-f74c00?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[c-lang]: https://img.shields.io/badge/C-00599C?style=for-the-badge&logo=c&logoColor=white
[C-url]: "https://en.wikipedia.org/wiki/C_(programming_language)"
[slint-lang]: https://img.shields.io/badge/Slint-7F52FF?style=for-the-badge&logo=slint&logoColor=white
[Slint-url]: https://slint-ui.com/
[jabcode-badge]: https://img.shields.io/badge/JABCode-00eded?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB2ZXJzaW9uPSIxLjIiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDI1IDI1IiB3aWR0aD0iMjUiIGhlaWdodD0iMjUiPgoJPHRpdGxlPmphYmNvZGVfYmFkZ2U8L3RpdGxlPgoJPHN0eWxlPgoJCS5zMCB7IGZpbGw6ICNmZmZmZmYgfSAKCTwvc3R5bGU+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0wIDE1di0xNWgxNXY1aC0xMHYxMHoiLz4KCTxwYXRoIGlkPSJQYXRoIDAiIGNsYXNzPSJzMCIgZD0ibTYgMTV2LTloOXYzaC02djZ6Ii8+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0xMCAxNXYtNWg1djV6Ii8+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0xMCAxNmg2di02aDN2OWgtOXoiLz4KCTxwYXRoIGlkPSJQYXRoIDAiIGNsYXNzPSJzMCIgZD0ibTEwIDIwaDEwdi0xMGg1djE1aC0xNXoiLz4KPC9zdmc+
[JABCode-url]: https://jabcode.org
[gcc-badge]: https://img.shields.io/badge/GCC-4E9A06?style=for-the-badge&logo=gnu&logoColor=white
[GCC-url]: https://gcc.gnu.org/
