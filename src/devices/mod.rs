use anyhow;
mod hyperxduocast;
mod lianliinf;

pub fn setup() -> anyhow::Result<()> {
    lianliinf::setup()?;
    hyperxduocast::setup()?;

    Ok(())
}
