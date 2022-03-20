use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem},
    Frame,
};

use crate::{
    economy::{Building, Economy, Season},
    nihilists::Nihilists,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, economy: &Economy, nils: &Nihilists) {
    // Screen
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Percentage(30),
        ])
        .split(frame.size());

    // TOP
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[0]);

    //------ Calendar ------------------------------------------------------------------------------
    let season = Season::from(economy.day);
    frame.render_widget(
        Block::default()
            .title(format!("📆 Day {} - {}", economy.day, season))
            .borders(Borders::ALL),
        top_layout[0],
    );

    let calender_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(12)])
        .split(top_layout[0]);

    let mut list_items = vec![];

    // Population
    list_items.push(ListItem::new(format!(
        "👪 Population  {}",
        economy.population
    )));

    // Population Cap
    list_items.push(ListItem::new(format!(
        "🏠 Housing     {}",
        economy.population_cap
    )));

    // Population Cap
    list_items.push(ListItem::new(format!(
        "🏭 Efficiency  {:.1}%",
        economy.efficiency * 100.0
    )));

    frame.render_widget(List::new(list_items), calender_layout[0]);

    //----------------------------------------------------------------------------------------------

    //------ Storage -------------------------------------------------------------------------------
    frame.render_widget(
        Block::default().title("📦 Storage ").borders(Borders::ALL),
        top_layout[1],
    );

    let storage_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(top_layout[1]);

    let food_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::LightRed))
        .label(format!("🍖 {:>4}/{}", economy.food, economy.storage))
        .percent((100 * economy.food / economy.storage) as u16);
    frame.render_widget(food_gauge, storage_layout[0]);

    let wood_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::LightGreen))
        .label(format!("🪵 {:>4}/{}", economy.wood, economy.storage))
        .percent((100 * economy.wood / economy.storage) as u16);
    frame.render_widget(wood_gauge, storage_layout[1]);

    let stone_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Gray))
        .label(format!("🪨 {:>4}/{}", economy.stone, economy.storage))
        .percent((100 * economy.stone / economy.storage) as u16);
    frame.render_widget(stone_gauge, storage_layout[2]);

    let iron_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::LightYellow))
        .label(format!("🪙 {:>4}/{}", economy.iron, economy.storage))
        .percent((100 * economy.iron / economy.storage) as u16);
    frame.render_widget(iron_gauge, storage_layout[3]);
    //----------------------------------------------------------------------------------------------

    //------ Nihilists -----------------------------------------------------------------------------
    frame.render_widget(
        Block::default()
            .title("🚩 Nihilists 👺")
            .borders(Borders::ALL),
        layout[1],
    );

    let mut nihilists_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    nihilists_layout[0].x += 1;
    nihilists_layout[0].y += 1;

    nihilists_layout[0].width -= 1;
    nihilists_layout[0].height -= 1;

    // Overall
    frame.render_widget(
        List::new(vec![
            // Undercover
            ListItem::new(format!("  🥸 Undercover {:>3}", nils.undercover)),
            // Recruiters
            ListItem::new(format!("  🤝 Recruiters {:>3}", nils.recruiters)),
            // Vee cur off your Johnson
            ListItem::new(format!("  🔪 Hitmen {:>7}", nils.hitmen)),
            // Efficiency§
            ListItem::new(format!("  ⚙️  Efficiency {:.1}%", nils.efficiency * 100.0)),
        ]),
        nihilists_layout[0],
    );

    for (i, building) in Building::iter().enumerate() {
        frame.render_widget(
            List::new(vec![
                ListItem::new(format!(
                    "  📣 Agitators {}",
                    nils.agitators.get(&building).unwrap()
                )),
                ListItem::new(format!(
                    "  🧨 Saboteurs {}",
                    nils.saboteurs.get(&building).unwrap()
                )),
                ListItem::new(format!(
                    "  💰 Embezzlers {}",
                    nils.embezzlers.get(&building).unwrap()
                )),
            ])
            .block(
                Block::default()
                    .title(building.to_string())
                    .borders(Borders::ALL),
            ),
            nihilists_layout[i + 1],
        );
    }
    //----------------------------------------------------------------------------------------------

    frame.render_widget(
        Block::default().title("Headlines").borders(Borders::ALL),
        layout[2],
    );
}
