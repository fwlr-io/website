/// THEMES

const themeDefs = {
  atelierDune: {
    dark: {
      darkBackground: "#20201d",
      mediumBackground: "#292824",
      lightBackground: "#6e6b53",
      superlightBackground: "#7d7a68",
      darkForeground: "#999580",
      mediumForeground: "#a6a28c",
      lightForeground: "#e8e4cf",
      superlightForeground: "#fefbec",

      yellow: "#ae9513",
      orange: "#b65611",
      red: "#d73737",
      magenta: "#d43552",
      violet: "#b854d4",
      blue: "#6684e1",
      cyan: "#1fad83",
      green: "#60ac39",
    },

    light: {
      darkBackground: "#fefbec",
      mediumBackground: "#e8e4cf",
      lightBackground: "#a6a28c",
      superlightBackground: "#999580",
      darkForeground: "#7d7a68",
      mediumForeground: "#6e6b5e",
      lightForeground: "#292824",
      superlightForeground: "#20201d",

      yellow: "#ae9513",
      orange: "#b65611",
      red: "#d73737",
      magenta: "#d43552",
      violet: "#b854d4",
      blue: "#6684e1",
      cyan: "#1fad83",
      green: "#60ac39",
    },
  },

  solarized: {
    dark: {
      darkBackground: "#002b36",
      mediumBackground: "#073642",
      lightBackground: "#586e75",
      superlightBackground: "#657b83",
      darkForeground: "#839496",
      mediumForeground: "#93a1a1",
      lightForeground: "#eee8d5",
      superlightForeground: "#fdf6e3",

      yellow: "#b58900",
      orange: "#cb4b16",
      red: "#dc322f",
      magenta: "#d33682",
      violet: "#6c71c4",
      blue: "#268bd2",
      cyan: "#2aa198",
      green: "#859900",
    },

    light: {
      darkBackground: "#fdf6e3",
      mediumBackground: "#eee8d5",
      lightBackground: "#93a1a1",
      superlightBackground: "#839496",
      darkForeground: "#657b83",
      mediumForeground: "#586e75",
      lightForeground: "#073642",
      superlightForeground: "#002b36",

      yellow: "#b58900",
      orange: "#cb4b16",
      red: "#dc322f",
      magenta: "#d33682",
      violet: "#6c71c4",
      blue: "#268bd2",
      cyan: "#2aa198",
      green: "#859900",
    },
  },
}

export const darkBackground = (props) => props.theme.darkBackground
export const mediumBackground = (props) => props.theme.mediumBackground
export const lightBackground = (props) => props.theme.lightBackground
export const superlightBackground = (props) => props.theme.superlightBackground
export const darkForeground = (props) => props.theme.darkForeground
export const mediumForeground = (props) => props.theme.mediumForeground
export const lightForeground = (props) => props.theme.lightForeground
export const superlightForeground = (props) => props.theme.superlightForeground

export const yellow = (props) => props.theme.yellow
export const orange = (props) => props.theme.orange
export const red = (props) => props.theme.red
export const magenta = (props) => props.theme.magenta
export const violet = (props) => props.theme.violet
export const blue = (props) => props.theme.blue
export const cyan = (props) => props.theme.cyan
export const green = (props) => props.theme.green

export default themeDefs

// / THEME FORMAT
//
// const themeFormat = {
//   name: "",
//
//   dark: {
//     darkBackground: "",
//     mediumBackground: "",
//     lightBackground: "",
//     superlightBackground: "",
//     darkForeground: "",
//     mediumForeground: "",
//     lightForeground: "",
//     superlightForeground: "",
//
//     yellow: "",
//     orange: "",
//     red: "",
//     magenta: "",
//     violet: "",
//     blue: "",
//     cyan: "",
//     green: "",
//   },
//   light: {
//     darkBackground: "",
//     mediumBackground: "",
//     lightBackground: "",
//     superlightBackground: "",
//     darkForeground: "",
//     mediumForeground: "",
//     lightForeground: "",
//     superlightForeground: "",
//
//     yellow: "",
//     orange: "",
//     red: "",
//     magenta: "",
//     violet: "",
//     blue: "",
//     cyan: "",
//     green: "",
//   },
// }
