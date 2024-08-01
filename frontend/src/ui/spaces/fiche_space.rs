use std::sync::{Arc, RwLock};

use eframe::egui;
use eframe::emath::Align;
use eframe::epaint::{Margin, Rounding};
use egui::{hex_color, Layout};
use egui_commonmark::CommonMarkCache;

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, Job};
use shared::fiche_rp::ScienceLevel::Senior;
use shared::fiche_rp::ScienceRole::Researcher;

use crate::ui::components_helper::{ficherp_bubble, ficherp_viewer};

pub struct FicheSpace {
    pub cache: Arc<RwLock<CommonMarkCache>>,
}
impl FicheSpace {
    pub fn new() -> Self {
        FicheSpace {
            cache: Arc::new(RwLock::new(CommonMarkCache::default()))
        }
    }
}

impl eframe::App for FicheSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ficherp = FicheRP {
                name: "Richard DuComposte".to_string(),
                job: Job::Science(Researcher(Senior)),
                description: "1m70\nyeux verts\ncravate avec un petit nœud désopilant\ncaucasien blanc\nrelativement discret, attire peu l'attention\npossède une calvitie lui donnant un crâne immaculé".to_string(),
                lore: "Il est un chercheur diplômé en physique théorique. Avant de rejoindre la Fondation, il travaillait au sein d'une prestigieuse université. Sa curiosité avide l'avait conduit à explorer des domaines de recherche souvent considérés comme marginaux par ses pairs. Au cours de ses travaux, il a découvert des propriétés anormales dans certains phénomènes physiques, notamment des fluctuations inexpliquées dans des champs magnétiques. Fasciné par ces anomalies, il a décidé de consacrer une thèse entière à ces découvertes.

Voyant une opportunité d'obtenir un arrangement gagnant-gagnant, la Fondation a pris des mesures pour dépublier la thèse et a proposé au chercheur de rejoindre ses rangs. Étant avide de curiosité, il a été immédiatement séduit par l'idée de travailler au sein de la Fondation.

Il a commencé sa carrière à la Fondation en tant qu'assistant, participant à de nombreuses expériences, notamment sur les objets SCP présents sur le site 65, principalement en raison de leur faible niveau d'accréditation requis et, pour la majorité, de leur inoffensivité avec les bonnes précautions. Il a ensuite assisté à quelques expériences sur d'autres SCP, notamment le classique nettoyage de SCP-173.

Il a ensuite entrepris ses propres expériences en tant que scientifique sur les objets SCP, puis sur des SCP plus complexes comme SCP-860 pour y vérifier les principes physiques de sa dimension, qu'il n'a malheureusement pas pu continuer. Il a assisté à une expérience sur SCP-076 menée par le professeur Anthony, accompagné de Michaud. En voyant l'expérience sombrer dans le drame suivi du déconfinement de ce dernier, cela lui a rappelé l'omniprésence du danger dans la Fondation. Peu de temps après, il s'est intéressé à SCP-966 et à leurs comportements en groupe (étude de leur organisation, possibilité d'organisation sociale). Les rapports ont malheureusement été perdus lors de la fermeture temporaire du Site-65. Peu avant la fermeture du site, il a assisté à deux expériences en compagnie d'autres chercheurs et scientifiques sur SCP-035. La première a été très lucrative en termes d'informations et très formatrice pour DuComposte. Cependant, la deuxième a été très étrange, SCP-035 déclarant être \"Baptiste, 28 ans et habitant à Paris\". Après quelques expériences sur SCP-023 et SCP-939, il a été promu chercheur et a effectué une simple \"interview\" avec SCP-035 à propos de son supposé passé avant la fermeture temporaire du site.

Lors de la réouverture, DuComposte a repris ses expériences habituelles, notamment sur les objets SCP comme SCP-009 avec des variantes d'eau (deutérium et tritium), ou encore des études des capacités sensorielles de SCP-023. DuComposte a aussi interagi avec SCP-049 de manière très cordiale, ne cherchant pas forcément à attirer l'attention sur lui, lui donnant simplement des Classe-D pour ses recherches (changement de tempérament). Après de courtes vacances bouleversées par la rébellion des [DONNÉES SUPPRIMÉES], il a fait sa rentrée sur le nouveau site avec les autres scientifiques du Site-27. Il a alors entamé une série d'expériences pour étudier les effets de SCP-1074. La première s'intéressait aux effets du SCP sur les personnes porteuses de handicap, plus précisément aux personnes aveugles. Une deuxième s'intéressait aux personnes porteuses de maladies neurodégénératives (ou en cas de tumeurs au cerveau) pour étudier les limites de l'influence de SCP-1074. Celle-ci a malheureusement été suspendue avec la fermeture du Site-27 à la suite des événements de [DONNÉES SUPPRIMÉES].#".to_string(),
                submission_date: 1722523698,
                messages: vec![],
                version: vec![],
                state: FicheState::Waiting,
            };
            let user = User {
                id: "374283393799553036".to_string(),
                username: "leskitou".to_string(),
                avatar: "80175a9ae4eb43805da7b3df1561fda0".to_string(),
            };

            let frame = egui::Frame::none()
                .fill(hex_color!("262626"))
                .rounding(Rounding {
                    nw: 25.0,
                    ne: 25.0,
                    sw: 25.0,
                    se: 25.0,
                })
                .inner_margin(Margin {
                    left: 10.0,
                    right: 10.0,
                    top: 10.0,
                    bottom: 10.0,
                }).outer_margin(Margin {
                left: 5.0,
                right: 5.0,
                top: 0.0,
                bottom: 10.0,
            });

            ui.columns(3, |mut columns| {
                columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.centered_and_justified(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical(|ui| {
                                egui::menu::bar(ui, |ui| {
                                    //TODO: FILTERING
                                });

                                for i in 0..50 {
                                    frame.show(ui, |ui| {
                                        ficherp_bubble(ui, &ficherp, &user);
                                    });
                                }
                            });
                        });
                    });
                });

                columns[1].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            frame.show(ui, |ui| {
                                ficherp_viewer(ui, &ficherp, &user, self.cache.clone());
                            });
                            frame.show(ui, |ui| {
                                //ficherp_viewer(ui, &ficherp, &user);
                            });
                        });
                    });
                });
                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.centered_and_justified(|ui| {});
                });
            });
        });
    }
}