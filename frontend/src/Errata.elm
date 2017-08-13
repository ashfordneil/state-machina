module Errata exposing (..)

import Html exposing (text, div, p, a, ul, li, h2, h3)
import Html.Attributes exposing (href, target)

stateMachines =
    div []
    [ 
        h2 [] [text "Information"],
        h3 [] [text "About this application"],
        p []
        [ text
            ("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod"
                ++ " tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis"
                ++ " nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                ++ " Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu"
                ++ " fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in"
                ++ " culpa qui officia deserunt mollit anim id est laborum."
            )
        ],
        h3 [] [text "Resources"],
        ul []
        [
            li []
            [
                a
                [
                    href "http://blog.markshead.com/869/state-machines-computer-science/",
                    target "_blank"
                ]
                [
                    text "State Machines - Basics of Comuter Science"
                ]
            ]
        ]
    ]
